use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 113, 215, 14], OperandSize::Dword)
}

#[test]
fn vpsrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 113, 211, 48], OperandSize::Qword)
}

#[test]
fn vpsrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 113, 210, 13], OperandSize::Dword)
}

#[test]
fn vpsrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 113, 211, 113], OperandSize::Qword)
}

#[test]
fn vpsrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 113, 214, 40], OperandSize::Dword)
}

#[test]
fn vpsrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 1073393563, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 113, 150, 155, 175, 250, 63, 47], OperandSize::Dword)
}

#[test]
fn vpsrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM22)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 77, 143, 113, 214, 96], OperandSize::Qword)
}

#[test]
fn vpsrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 904059876, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 131, 113, 148, 177, 228, 219, 226, 53, 80], OperandSize::Qword)
}

#[test]
fn vpsrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 171, 113, 208, 123], OperandSize::Dword)
}

#[test]
fn vpsrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 113, 20, 113, 39], OperandSize::Dword)
}

#[test]
fn vpsrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM9)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 37, 166, 113, 209, 34], OperandSize::Qword)
}

#[test]
fn vpsrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM25)), operand2: Some(IndirectDisplaced(RBX, 2141966796, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 53, 164, 113, 147, 204, 209, 171, 127, 18], OperandSize::Qword)
}

#[test]
fn vpsrlw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 113, 210, 88], OperandSize::Dword)
}

#[test]
fn vpsrlw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 113, 20, 242, 72], OperandSize::Dword)
}

#[test]
fn vpsrlw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 125, 196, 113, 210, 63], OperandSize::Qword)
}

#[test]
fn vpsrlw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM15)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 5, 204, 113, 16, 35], OperandSize::Qword)
}

#[test]
fn vpsrlw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 209, 223], OperandSize::Dword)
}

#[test]
fn vpsrlw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 209, 24], OperandSize::Dword)
}

#[test]
fn vpsrlw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 209, 245], OperandSize::Qword)
}

#[test]
fn vpsrlw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 209, 32], OperandSize::Qword)
}

#[test]
fn vpsrlw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 209, 230], OperandSize::Dword)
}

#[test]
fn vpsrlw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 209, 4, 146], OperandSize::Dword)
}

#[test]
fn vpsrlw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 209, 246], OperandSize::Qword)
}

#[test]
fn vpsrlw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 209, 4, 113], OperandSize::Qword)
}

#[test]
fn vpsrlw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 209, 250], OperandSize::Dword)
}

#[test]
fn vpsrlw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 209, 34], OperandSize::Dword)
}

#[test]
fn vpsrlw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 5, 133, 209, 204], OperandSize::Qword)
}

#[test]
fn vpsrlw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 1519962105, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 85, 141, 209, 177, 249, 199, 152, 90], OperandSize::Qword)
}

#[test]
fn vpsrlw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 172, 209, 226], OperandSize::Dword)
}

#[test]
fn vpsrlw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 209, 20, 64], OperandSize::Dword)
}

#[test]
fn vpsrlw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 170, 209, 207], OperandSize::Qword)
}

#[test]
fn vpsrlw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 61, 169, 209, 49], OperandSize::Qword)
}

#[test]
fn vpsrlw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 206, 209, 232], OperandSize::Dword)
}

#[test]
fn vpsrlw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EAX, 675880216, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 209, 128, 24, 29, 73, 40], OperandSize::Dword)
}

#[test]
fn vpsrlw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 125, 198, 209, 249], OperandSize::Qword)
}

#[test]
fn vpsrlw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RSI, 402679971, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 61, 194, 209, 150, 163, 104, 0, 24], OperandSize::Qword)
}

