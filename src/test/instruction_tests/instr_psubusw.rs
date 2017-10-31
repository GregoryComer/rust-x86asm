use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 254], OperandSize::Dword)
}

#[test]
fn psubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDI, 1670248176, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 191, 240, 246, 141, 99], OperandSize::Dword)
}

#[test]
fn psubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 241], OperandSize::Qword)
}

#[test]
fn psubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1309455114, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 52, 221, 10, 179, 12, 78], OperandSize::Qword)
}

#[test]
fn psubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 214], OperandSize::Dword)
}

#[test]
fn psubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1555776112, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 188, 136, 112, 66, 187, 92], OperandSize::Dword)
}

#[test]
fn psubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 243], OperandSize::Qword)
}

#[test]
fn psubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 786588994, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 168, 66, 101, 226, 46], OperandSize::Qword)
}

