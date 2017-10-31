use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 113, 210, 61], OperandSize::Dword)
}

#[test]
fn vpsrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 113, 211, 115], OperandSize::Qword)
}

#[test]
fn vpsrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 213, 42], OperandSize::Dword)
}

#[test]
fn vpsrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 210, 41], OperandSize::Qword)
}

#[test]
fn vpsrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 139, 113, 210, 26], OperandSize::Dword)
}

#[test]
fn vpsrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 41397169, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 113, 20, 117, 177, 171, 119, 2, 60], OperandSize::Dword)
}

#[test]
fn vpsrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 5, 132, 113, 208, 13], OperandSize::Qword)
}

#[test]
fn vpsrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1379609119, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 61, 133, 113, 20, 117, 31, 42, 59, 82, 10], OperandSize::Qword)
}

#[test]
fn vpsrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 113, 209, 125], OperandSize::Dword)
}

#[test]
fn vpsrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 2117005727, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 169, 113, 20, 125, 159, 241, 46, 126, 80], OperandSize::Dword)
}

#[test]
fn vpsrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 61, 172, 113, 211, 117], OperandSize::Qword)
}

#[test]
fn vpsrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1479251059, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 166, 113, 20, 157, 115, 148, 43, 88, 97], OperandSize::Qword)
}

#[test]
fn vpsrlw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 113, 208, 70], OperandSize::Dword)
}

#[test]
fn vpsrlw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1189042933, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 113, 20, 141, 245, 90, 223, 70, 53], OperandSize::Dword)
}

#[test]
fn vpsrlw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM11)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 101, 198, 113, 211, 14], OperandSize::Qword)
}

#[test]
fn vpsrlw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 534572745, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 113, 20, 253, 201, 238, 220, 31, 21], OperandSize::Qword)
}

#[test]
fn vpsrlw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 209, 229], OperandSize::Dword)
}

#[test]
fn vpsrlw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 308711132, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 209, 180, 186, 220, 142, 102, 18], OperandSize::Dword)
}

#[test]
fn vpsrlw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 209, 212], OperandSize::Qword)
}

#[test]
fn vpsrlw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 234837446, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 209, 52, 221, 198, 85, 255, 13], OperandSize::Qword)
}

#[test]
fn vpsrlw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 209, 253], OperandSize::Dword)
}

#[test]
fn vpsrlw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 2082263586, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 209, 180, 249, 34, 210, 28, 124], OperandSize::Dword)
}

#[test]
fn vpsrlw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 209, 247], OperandSize::Qword)
}

#[test]
fn vpsrlw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RBX, 733603324, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 209, 163, 252, 229, 185, 43], OperandSize::Qword)
}

#[test]
fn vpsrlw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 209, 242], OperandSize::Dword)
}

#[test]
fn vpsrlw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 209, 6], OperandSize::Dword)
}

#[test]
fn vpsrlw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 61, 129, 209, 200], OperandSize::Qword)
}

#[test]
fn vpsrlw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 347371036, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 5, 134, 209, 156, 114, 28, 118, 180, 20], OperandSize::Qword)
}

#[test]
fn vpsrlw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 209, 243], OperandSize::Dword)
}

#[test]
fn vpsrlw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 672763651, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 209, 188, 215, 3, 143, 25, 40], OperandSize::Dword)
}

#[test]
fn vpsrlw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 45, 161, 209, 222], OperandSize::Qword)
}

#[test]
fn vpsrlw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 29, 164, 209, 10], OperandSize::Qword)
}

#[test]
fn vpsrlw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 209, 247], OperandSize::Dword)
}

#[test]
fn vpsrlw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 209, 36, 66], OperandSize::Dword)
}

#[test]
fn vpsrlw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 13, 202, 209, 244], OperandSize::Qword)
}

#[test]
fn vpsrlw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 2146609883, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 45, 195, 209, 132, 67, 219, 170, 242, 127], OperandSize::Qword)
}

