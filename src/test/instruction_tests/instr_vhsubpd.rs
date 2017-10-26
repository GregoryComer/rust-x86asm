use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 125, 232], OperandSize::Dword)
}

#[test]
fn vhsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 780636125, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 125, 186, 221, 143, 135, 46], OperandSize::Dword)
}

#[test]
fn vhsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 125, 232], OperandSize::Qword)
}

#[test]
fn vhsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1769203228, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 125, 20, 93, 28, 230, 115, 105], OperandSize::Qword)
}

#[test]
fn vhsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 125, 253], OperandSize::Dword)
}

#[test]
fn vhsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 1890816872, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 125, 135, 104, 147, 179, 112], OperandSize::Dword)
}

#[test]
fn vhsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 125, 208], OperandSize::Qword)
}

#[test]
fn vhsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 862894753, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 125, 172, 187, 161, 186, 110, 51], OperandSize::Qword)
}

