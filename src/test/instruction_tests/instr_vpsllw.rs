use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 113, 244, 4], OperandSize::Dword)
}

#[test]
fn vpsllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 113, 243, 28], OperandSize::Qword)
}

#[test]
fn vpsllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 113, 246, 69], OperandSize::Dword)
}

#[test]
fn vpsllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 113, 242, 124], OperandSize::Qword)
}

#[test]
fn vpsllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 113, 245, 120], OperandSize::Dword)
}

#[test]
fn vpsllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1548276062, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 113, 180, 194, 94, 209, 72, 92, 109], OperandSize::Dword)
}

#[test]
fn vpsllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 77, 138, 113, 244, 27], OperandSize::Qword)
}

#[test]
fn vpsllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 817131168, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 113, 52, 197, 160, 110, 180, 48, 38], OperandSize::Qword)
}

#[test]
fn vpsllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 113, 247, 14], OperandSize::Dword)
}

#[test]
fn vpsllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1730680081, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 172, 113, 52, 117, 17, 21, 40, 103, 89], OperandSize::Dword)
}

#[test]
fn vpsllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 117, 163, 113, 240, 59], OperandSize::Qword)
}

#[test]
fn vpsllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 5, 174, 113, 52, 207, 36], OperandSize::Qword)
}

#[test]
fn vpsllw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 113, 243, 10], OperandSize::Dword)
}

#[test]
fn vpsllw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1784313502, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 203, 113, 52, 205, 158, 118, 90, 106, 107], OperandSize::Dword)
}

#[test]
fn vpsllw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 113, 245, 116], OperandSize::Qword)
}

#[test]
fn vpsllw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectDisplaced(RDI, 1995174128, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 194, 113, 183, 240, 240, 235, 118, 126], OperandSize::Qword)
}

#[test]
fn vpsllw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 241, 225], OperandSize::Dword)
}

#[test]
fn vpsllw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 319943354, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 241, 145, 186, 242, 17, 19], OperandSize::Dword)
}

#[test]
fn vpsllw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 241, 232], OperandSize::Qword)
}

#[test]
fn vpsllw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 241, 60, 113], OperandSize::Qword)
}

#[test]
fn vpsllw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 241, 252], OperandSize::Dword)
}

#[test]
fn vpsllw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 469569507, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 241, 12, 117, 227, 15, 253, 27], OperandSize::Dword)
}

#[test]
fn vpsllw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 241, 212], OperandSize::Qword)
}

#[test]
fn vpsllw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 241, 44, 190], OperandSize::Qword)
}

#[test]
fn vpsllw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 241, 206], OperandSize::Dword)
}

#[test]
fn vpsllw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 313025878, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 241, 187, 86, 101, 168, 18], OperandSize::Dword)
}

#[test]
fn vpsllw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 109, 133, 241, 250], OperandSize::Qword)
}

#[test]
fn vpsllw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 785898092, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 117, 131, 241, 180, 192, 108, 218, 215, 46], OperandSize::Qword)
}

#[test]
fn vpsllw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 241, 219], OperandSize::Dword)
}

#[test]
fn vpsllw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 241, 4, 80], OperandSize::Dword)
}

#[test]
fn vpsllw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 5, 169, 241, 238], OperandSize::Qword)
}

#[test]
fn vpsllw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RDI, 1579308628, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 5, 172, 241, 151, 84, 86, 34, 94], OperandSize::Qword)
}

#[test]
fn vpsllw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 204, 241, 201], OperandSize::Dword)
}

#[test]
fn vpsllw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1676190838, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 241, 12, 125, 118, 164, 232, 99], OperandSize::Dword)
}

#[test]
fn vpsllw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 37, 195, 241, 204], OperandSize::Qword)
}

#[test]
fn vpsllw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 125, 203, 241, 58], OperandSize::Qword)
}

