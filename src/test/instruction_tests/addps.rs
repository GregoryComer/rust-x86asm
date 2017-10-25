use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn addps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 238], OperandSize::Dword)
}

fn addps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 4, 145], OperandSize::Dword)
}

fn addps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 209], OperandSize::Qword)
}

fn addps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 88, 23], OperandSize::Qword)
}

