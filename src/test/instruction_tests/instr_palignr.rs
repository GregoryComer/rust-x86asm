use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn palignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 217, 63], OperandSize::Dword)
}

#[test]
fn palignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 539923898, Some(OperandSize::Qword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 52, 157, 186, 149, 46, 32, 14], OperandSize::Dword)
}

#[test]
fn palignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM2)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 218, 89], OperandSize::Qword)
}

#[test]
fn palignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 34, 29], OperandSize::Qword)
}

#[test]
fn palignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 235, 74], OperandSize::Dword)
}

#[test]
fn palignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 36, 158, 1], OperandSize::Dword)
}

#[test]
fn palignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 192, 37], OperandSize::Qword)
}

#[test]
fn palignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 1314826731, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 177, 235, 169, 94, 78, 3], OperandSize::Qword)
}

