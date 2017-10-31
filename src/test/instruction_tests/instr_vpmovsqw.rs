use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 36, 232], OperandSize::Dword)
}

#[test]
fn vpmovsqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 36, 47], OperandSize::Dword)
}

#[test]
fn vpmovsqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 126, 143, 36, 244], OperandSize::Qword)
}

#[test]
fn vpmovsqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(RBX, 818213665, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 36, 147, 33, 243, 196, 48], OperandSize::Qword)
}

#[test]
fn vpmovsqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 36, 245], OperandSize::Dword)
}

#[test]
fn vpmovsqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1685601502, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 4, 181, 222, 60, 120, 100], OperandSize::Dword)
}

#[test]
fn vpmovsqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 126, 175, 36, 210], OperandSize::Qword)
}

#[test]
fn vpmovsqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(RCX, 1421847352, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 185, 56, 171, 191, 84], OperandSize::Qword)
}

#[test]
fn vpmovsqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 36, 212], OperandSize::Dword)
}

#[test]
fn vpmovsqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(ECX, 1987831965, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 169, 157, 232, 123, 118], OperandSize::Dword)
}

#[test]
fn vpmovsqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM14)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 126, 203, 36, 246], OperandSize::Qword)
}

#[test]
fn vpmovsqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(RDI, 1006604683, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 36, 167, 139, 145, 255, 59], OperandSize::Qword)
}

