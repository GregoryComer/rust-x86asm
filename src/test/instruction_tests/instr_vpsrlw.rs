use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 113, 213, 97], OperandSize::Dword)
}

#[test]
fn vpsrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 113, 214, 99], OperandSize::Qword)
}

#[test]
fn vpsrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 214, 33], OperandSize::Dword)
}

#[test]
fn vpsrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 113, 213, 16], OperandSize::Qword)
}

#[test]
fn vpsrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 113, 208, 111], OperandSize::Dword)
}

#[test]
fn vpsrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1480474247, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 140, 113, 148, 215, 135, 62, 62, 88, 68], OperandSize::Dword)
}

#[test]
fn vpsrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM15)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 37, 135, 113, 215, 56], OperandSize::Qword)
}

#[test]
fn vpsrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1378751725, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 13, 139, 113, 148, 134, 237, 20, 46, 82, 14], OperandSize::Qword)
}

#[test]
fn vpsrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 113, 213, 9], OperandSize::Dword)
}

#[test]
fn vpsrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EBX, 1892279091, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 113, 147, 51, 227, 201, 112, 87], OperandSize::Dword)
}

#[test]
fn vpsrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM12)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 109, 163, 113, 212, 107], OperandSize::Qword)
}

#[test]
fn vpsrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2013745950, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 37, 165, 113, 20, 157, 30, 83, 7, 120, 52], OperandSize::Qword)
}

#[test]
fn vpsrlw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 113, 214, 68], OperandSize::Dword)
}

#[test]
fn vpsrlw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 113, 20, 144, 99], OperandSize::Dword)
}

#[test]
fn vpsrlw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 109, 198, 113, 215, 7], OperandSize::Qword)
}

#[test]
fn vpsrlw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectDisplaced(RCX, 926127663, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 193, 113, 145, 47, 150, 51, 55, 29], OperandSize::Qword)
}

#[test]
fn vpsrlw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 209, 210], OperandSize::Dword)
}

#[test]
fn vpsrlw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1411184165, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 209, 143, 37, 246, 28, 84], OperandSize::Dword)
}

#[test]
fn vpsrlw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 209, 205], OperandSize::Qword)
}

#[test]
fn vpsrlw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RCX, 1582757264, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 209, 185, 144, 245, 86, 94], OperandSize::Qword)
}

#[test]
fn vpsrlw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 209, 226], OperandSize::Dword)
}

#[test]
fn vpsrlw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1776048408, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 209, 20, 149, 24, 89, 220, 105], OperandSize::Dword)
}

#[test]
fn vpsrlw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 209, 211], OperandSize::Qword)
}

#[test]
fn vpsrlw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1341772654, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 209, 164, 182, 110, 211, 249, 79], OperandSize::Qword)
}

#[test]
fn vpsrlw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 209, 233], OperandSize::Dword)
}

#[test]
fn vpsrlw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1066974297, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 209, 28, 149, 89, 188, 152, 63], OperandSize::Dword)
}

#[test]
fn vpsrlw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 129, 209, 203], OperandSize::Qword)
}

#[test]
fn vpsrlw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1272723224, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 13, 142, 209, 52, 221, 24, 55, 220, 75], OperandSize::Qword)
}

#[test]
fn vpsrlw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 209, 222], OperandSize::Dword)
}

#[test]
fn vpsrlw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1929362236, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 209, 140, 78, 60, 187, 255, 114], OperandSize::Dword)
}

#[test]
fn vpsrlw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 45, 161, 209, 219], OperandSize::Qword)
}

#[test]
fn vpsrlw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1770040612, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 209, 28, 157, 36, 173, 128, 105], OperandSize::Qword)
}

#[test]
fn vpsrlw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 209, 201], OperandSize::Dword)
}

#[test]
fn vpsrlw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 726946091, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 209, 60, 69, 43, 81, 84, 43], OperandSize::Dword)
}

#[test]
fn vpsrlw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 85, 199, 209, 192], OperandSize::Qword)
}

#[test]
fn vpsrlw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1242129308, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 53, 193, 209, 140, 115, 156, 99, 9, 74], OperandSize::Qword)
}

