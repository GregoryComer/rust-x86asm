use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn por_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 192], OperandSize::Dword)
}

#[test]
fn por_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 31], OperandSize::Dword)
}

#[test]
fn por_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 227], OperandSize::Qword)
}

#[test]
fn por_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDI, 627096279, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 235, 151, 215, 186, 96, 37], OperandSize::Qword)
}

#[test]
fn por_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 219], OperandSize::Dword)
}

#[test]
fn por_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ECX, 926547752, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 169, 40, 255, 57, 55], OperandSize::Dword)
}

#[test]
fn por_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 207], OperandSize::Qword)
}

#[test]
fn por_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 15944860, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 235, 44, 77, 156, 76, 243, 0], OperandSize::Qword)
}

