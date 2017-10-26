use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM6)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 246, 71], OperandSize::Dword)
}

#[test]
fn pslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM2)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 242, 37], OperandSize::Qword)
}

#[test]
fn pslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 246, 94], OperandSize::Dword)
}

#[test]
fn pslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 241, 11], OperandSize::Qword)
}

#[test]
fn pslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 227], OperandSize::Dword)
}

#[test]
fn pslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 12, 151], OperandSize::Dword)
}

#[test]
fn pslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 255], OperandSize::Qword)
}

#[test]
fn pslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 335052561, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 242, 164, 191, 17, 127, 248, 19], OperandSize::Qword)
}

#[test]
fn pslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 238], OperandSize::Dword)
}

#[test]
fn pslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 47], OperandSize::Dword)
}

#[test]
fn pslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 204], OperandSize::Qword)
}

#[test]
fn pslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 242, 36, 155], OperandSize::Qword)
}

