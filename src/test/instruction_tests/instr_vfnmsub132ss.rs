use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 159, 247], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 159, 11], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 159, 231], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 2017008888, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 159, 155, 248, 28, 57, 120], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 249, 159, 208], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1623832151, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 159, 140, 143, 87, 182, 201, 96], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 77, 250, 159, 224], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1968988522, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 5, 137, 159, 132, 139, 106, 97, 92, 117], OperandSize::Qword)
}

