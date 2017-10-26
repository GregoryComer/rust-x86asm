use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 113, 213, 59], OperandSize::Dword)
}

#[test]
fn vpsrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 113, 209, 41], OperandSize::Qword)
}

#[test]
fn vpsrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 113, 208, 49], OperandSize::Dword)
}

#[test]
fn vpsrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 113, 208, 39], OperandSize::Qword)
}

#[test]
fn vpsrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 113, 209, 57], OperandSize::Dword)
}

#[test]
fn vpsrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 302037102, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 113, 20, 149, 110, 184, 0, 18, 75], OperandSize::Dword)
}

#[test]
fn vpsrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 5, 138, 113, 212, 25], OperandSize::Qword)
}

#[test]
fn vpsrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 42095908, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 21, 141, 113, 148, 186, 36, 85, 130, 2, 72], OperandSize::Qword)
}

#[test]
fn vpsrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 113, 214, 126], OperandSize::Dword)
}

#[test]
fn vpsrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 113, 20, 209, 110], OperandSize::Dword)
}

#[test]
fn vpsrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM27)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 69, 161, 113, 211, 119], OperandSize::Qword)
}

#[test]
fn vpsrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RBX, 1447450018, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 113, 147, 162, 85, 70, 86, 75], OperandSize::Qword)
}

#[test]
fn vpsrlw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 113, 209, 120], OperandSize::Dword)
}

#[test]
fn vpsrlw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EDX, 1025546897, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 113, 146, 145, 154, 32, 61, 52], OperandSize::Dword)
}

#[test]
fn vpsrlw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM14)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 61, 204, 113, 214, 101], OperandSize::Qword)
}

#[test]
fn vpsrlw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 113, 16, 112], OperandSize::Qword)
}

#[test]
fn vpsrlw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 209, 196], OperandSize::Dword)
}

#[test]
fn vpsrlw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 919456839, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 209, 52, 189, 71, 204, 205, 54], OperandSize::Dword)
}

#[test]
fn vpsrlw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 209, 221], OperandSize::Qword)
}

#[test]
fn vpsrlw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1384078286, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 209, 138, 206, 91, 127, 82], OperandSize::Qword)
}

#[test]
fn vpsrlw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 209, 205], OperandSize::Dword)
}

#[test]
fn vpsrlw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 2020714228, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 209, 152, 244, 166, 113, 120], OperandSize::Dword)
}

#[test]
fn vpsrlw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 209, 212], OperandSize::Qword)
}

#[test]
fn vpsrlw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 857650932, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 209, 156, 159, 244, 182, 30, 51], OperandSize::Qword)
}

#[test]
fn vpsrlw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 209, 209], OperandSize::Dword)
}

#[test]
fn vpsrlw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 209, 47], OperandSize::Dword)
}

#[test]
fn vpsrlw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 101, 141, 209, 227], OperandSize::Qword)
}

#[test]
fn vpsrlw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 435520261, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 117, 135, 209, 172, 179, 5, 131, 245, 25], OperandSize::Qword)
}

#[test]
fn vpsrlw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 209, 253], OperandSize::Dword)
}

#[test]
fn vpsrlw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1733427952, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 209, 175, 240, 2, 82, 103], OperandSize::Dword)
}

#[test]
fn vpsrlw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 5, 165, 209, 193], OperandSize::Qword)
}

#[test]
fn vpsrlw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 61, 172, 209, 12, 144], OperandSize::Qword)
}

#[test]
fn vpsrlw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 207, 209, 237], OperandSize::Dword)
}

#[test]
fn vpsrlw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1059085029, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 209, 132, 130, 229, 90, 32, 63], OperandSize::Dword)
}

#[test]
fn vpsrlw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 29, 204, 209, 235], OperandSize::Qword)
}

#[test]
fn vpsrlw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RAX, 837703386, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 205, 209, 144, 218, 86, 238, 49], OperandSize::Qword)
}

