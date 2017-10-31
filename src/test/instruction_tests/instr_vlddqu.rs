use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vlddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1256697189, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 164, 191, 101, 173, 231, 74], OperandSize::Dword)
}

#[test]
fn vlddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 52, 223], OperandSize::Qword)
}

#[test]
fn vlddqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 1], OperandSize::Dword)
}

#[test]
fn vlddqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1662312954, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 20, 125, 250, 225, 20, 99], OperandSize::Qword)
}

