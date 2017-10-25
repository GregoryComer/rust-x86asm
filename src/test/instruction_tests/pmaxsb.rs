use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 212], OperandSize::Dword)
}

fn pmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1663225016, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 36, 253, 184, 204, 34, 99], OperandSize::Dword)
}

fn pmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 208], OperandSize::Qword)
}

fn pmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 12647877, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 132, 177, 197, 253, 192, 0], OperandSize::Qword)
}

