use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 113, 245, 79], OperandSize::Dword)
}

#[test]
fn vpsllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 113, 241, 3], OperandSize::Qword)
}

#[test]
fn vpsllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 246, 57], OperandSize::Dword)
}

#[test]
fn vpsllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 246, 38], OperandSize::Qword)
}

#[test]
fn vpsllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 137, 113, 241, 44], OperandSize::Dword)
}

#[test]
fn vpsllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 1732696365, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 113, 178, 45, 217, 70, 103, 96], OperandSize::Dword)
}

#[test]
fn vpsllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 5, 133, 113, 242, 53], OperandSize::Qword)
}

#[test]
fn vpsllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM14)), operand2: Some(IndirectDisplaced(RDI, 984544681, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 13, 142, 113, 183, 169, 245, 174, 58, 110], OperandSize::Qword)
}

#[test]
fn vpsllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 113, 245, 28], OperandSize::Dword)
}

#[test]
fn vpsllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1406908982, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 113, 180, 67, 54, 186, 219, 83, 105], OperandSize::Dword)
}

#[test]
fn vpsllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 53, 165, 113, 246, 115], OperandSize::Qword)
}

#[test]
fn vpsllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM12)), operand2: Some(IndirectDisplaced(RBX, 1233583899, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 29, 171, 113, 179, 27, 255, 134, 73, 124], OperandSize::Qword)
}

#[test]
fn vpsllw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 113, 246, 119], OperandSize::Dword)
}

#[test]
fn vpsllw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 202, 113, 52, 176, 33], OperandSize::Dword)
}

#[test]
fn vpsllw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 61, 202, 113, 243, 27], OperandSize::Qword)
}

#[test]
fn vpsllw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 194, 113, 50, 104], OperandSize::Qword)
}

#[test]
fn vpsllw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 241, 201], OperandSize::Dword)
}

#[test]
fn vpsllw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1017445763, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 241, 147, 131, 253, 164, 60], OperandSize::Dword)
}

#[test]
fn vpsllw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 241, 218], OperandSize::Qword)
}

#[test]
fn vpsllw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1099943352, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 241, 28, 245, 184, 205, 143, 65], OperandSize::Qword)
}

#[test]
fn vpsllw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 241, 250], OperandSize::Dword)
}

#[test]
fn vpsllw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 860283302, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 241, 188, 185, 166, 225, 70, 51], OperandSize::Dword)
}

#[test]
fn vpsllw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 241, 203], OperandSize::Qword)
}

#[test]
fn vpsllw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 241, 52, 144], OperandSize::Qword)
}

#[test]
fn vpsllw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 137, 241, 217], OperandSize::Dword)
}

#[test]
fn vpsllw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 326877454, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 241, 156, 210, 14, 193, 123, 19], OperandSize::Dword)
}

#[test]
fn vpsllw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 61, 131, 241, 219], OperandSize::Qword)
}

#[test]
fn vpsllw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 37, 137, 241, 44, 126], OperandSize::Qword)
}

#[test]
fn vpsllw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 241, 213], OperandSize::Dword)
}

#[test]
fn vpsllw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 241, 35], OperandSize::Dword)
}

#[test]
fn vpsllw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 77, 161, 241, 199], OperandSize::Qword)
}

#[test]
fn vpsllw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 85, 171, 241, 36, 90], OperandSize::Qword)
}

#[test]
fn vpsllw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 241, 217], OperandSize::Dword)
}

#[test]
fn vpsllw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EDX, 1902273703, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 241, 138, 167, 100, 98, 113], OperandSize::Dword)
}

#[test]
fn vpsllw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 125, 196, 241, 246], OperandSize::Qword)
}

#[test]
fn vpsllw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 37, 203, 241, 36, 122], OperandSize::Qword)
}

