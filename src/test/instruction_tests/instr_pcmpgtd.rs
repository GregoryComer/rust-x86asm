use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 233], OperandSize::Dword)
}

#[test]
fn pcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1830469363, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 44, 205, 243, 190, 26, 109], OperandSize::Dword)
}

#[test]
fn pcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 204], OperandSize::Qword)
}

#[test]
fn pcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 28, 208], OperandSize::Qword)
}

#[test]
fn pcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 210], OperandSize::Dword)
}

#[test]
fn pcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 39], OperandSize::Dword)
}

#[test]
fn pcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 214], OperandSize::Qword)
}

#[test]
fn pcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 63], OperandSize::Qword)
}

