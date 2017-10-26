use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 113, 241, 102], OperandSize::Dword)
}

#[test]
fn vpsllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 113, 245, 44], OperandSize::Qword)
}

#[test]
fn vpsllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 113, 241, 99], OperandSize::Dword)
}

#[test]
fn vpsllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 240, 41], OperandSize::Qword)
}

#[test]
fn vpsllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 113, 242, 103], OperandSize::Dword)
}

#[test]
fn vpsllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 1703396528, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 113, 177, 176, 196, 135, 101, 122], OperandSize::Dword)
}

#[test]
fn vpsllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 5, 130, 113, 243, 65], OperandSize::Qword)
}

#[test]
fn vpsllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 270802084, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 29, 141, 113, 52, 213, 164, 28, 36, 16, 27], OperandSize::Qword)
}

#[test]
fn vpsllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 113, 244, 15], OperandSize::Dword)
}

#[test]
fn vpsllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 220743609, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 113, 180, 90, 185, 71, 40, 13, 35], OperandSize::Dword)
}

#[test]
fn vpsllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 61, 164, 113, 242, 11], OperandSize::Qword)
}

#[test]
fn vpsllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM17)), operand2: Some(IndirectDisplaced(RDX, 852170485, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 164, 113, 178, 245, 22, 203, 50, 105], OperandSize::Qword)
}

#[test]
fn vpsllw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 206, 113, 244, 89], OperandSize::Dword)
}

#[test]
fn vpsllw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1302137717, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 113, 180, 80, 117, 11, 157, 77, 40], OperandSize::Dword)
}

#[test]
fn vpsllw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 29, 193, 113, 242, 27], OperandSize::Qword)
}

#[test]
fn vpsllw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(RSI, 627255325, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 113, 182, 29, 40, 99, 37, 117], OperandSize::Qword)
}

#[test]
fn vpsllw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 241, 250], OperandSize::Dword)
}

#[test]
fn vpsllw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 241, 62], OperandSize::Dword)
}

#[test]
fn vpsllw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 241, 234], OperandSize::Qword)
}

#[test]
fn vpsllw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 241, 20, 126], OperandSize::Qword)
}

#[test]
fn vpsllw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 241, 233], OperandSize::Dword)
}

#[test]
fn vpsllw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 241, 52, 115], OperandSize::Dword)
}

#[test]
fn vpsllw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 241, 217], OperandSize::Qword)
}

#[test]
fn vpsllw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RBX, 1142113467, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 241, 155, 187, 68, 19, 68], OperandSize::Qword)
}

#[test]
fn vpsllw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 241, 210], OperandSize::Dword)
}

#[test]
fn vpsllw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1880219654, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 241, 28, 245, 6, 224, 17, 112], OperandSize::Dword)
}

#[test]
fn vpsllw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 69, 130, 241, 225], OperandSize::Qword)
}

#[test]
fn vpsllw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 37, 134, 241, 52, 198], OperandSize::Qword)
}

#[test]
fn vpsllw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 241, 232], OperandSize::Dword)
}

#[test]
fn vpsllw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 520131674, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 241, 156, 88, 90, 148, 0, 31], OperandSize::Dword)
}

#[test]
fn vpsllw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 93, 170, 241, 204], OperandSize::Qword)
}

#[test]
fn vpsllw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RSI, 1729585245, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 101, 162, 241, 150, 93, 96, 23, 103], OperandSize::Qword)
}

#[test]
fn vpsllw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 201, 241, 232], OperandSize::Dword)
}

#[test]
fn vpsllw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 241, 12, 179], OperandSize::Dword)
}

#[test]
fn vpsllw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 13, 198, 241, 245], OperandSize::Qword)
}

#[test]
fn vpsllw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 85, 198, 241, 25], OperandSize::Qword)
}

