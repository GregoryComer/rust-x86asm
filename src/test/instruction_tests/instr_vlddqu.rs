use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vlddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1707467354, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 188, 214, 90, 226, 197, 101], OperandSize::Dword)
}

#[test]
fn vlddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 52, 87], OperandSize::Qword)
}

#[test]
fn vlddqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 55], OperandSize::Dword)
}

#[test]
fn vlddqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 799013123, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 20, 197, 3, 249, 159, 47], OperandSize::Qword)
}

