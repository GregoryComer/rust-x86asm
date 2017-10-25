use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn hsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 234], OperandSize::Dword)
}

fn hsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 56], OperandSize::Dword)
}

fn hsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 212], OperandSize::Qword)
}

fn hsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1584422043, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 172, 113, 155, 92, 112, 94], OperandSize::Qword)
}

