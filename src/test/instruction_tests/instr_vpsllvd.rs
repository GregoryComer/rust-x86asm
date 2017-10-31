use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 71, 194], OperandSize::Dword)
}

#[test]
fn vpsllvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 71, 6], OperandSize::Dword)
}

#[test]
fn vpsllvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 71, 250], OperandSize::Qword)
}

#[test]
fn vpsllvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 71, 60, 240], OperandSize::Qword)
}

#[test]
fn vpsllvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 71, 217], OperandSize::Dword)
}

#[test]
fn vpsllvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1714446092, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 71, 132, 130, 12, 95, 48, 102], OperandSize::Dword)
}

#[test]
fn vpsllvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 71, 194], OperandSize::Qword)
}

#[test]
fn vpsllvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 71, 27], OperandSize::Qword)
}

#[test]
fn vpsllvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 71, 243], OperandSize::Dword)
}

#[test]
fn vpsllvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 2119827844, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 71, 12, 189, 132, 1, 90, 126], OperandSize::Dword)
}

#[test]
fn vpsllvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 49110169, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 157, 71, 36, 93, 153, 92, 237, 2], OperandSize::Dword)
}

#[test]
fn vpsllvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 29, 140, 71, 194], OperandSize::Qword)
}

#[test]
fn vpsllvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 45, 141, 71, 60, 122], OperandSize::Qword)
}

#[test]
fn vpsllvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 154, 71, 20, 87], OperandSize::Qword)
}

#[test]
fn vpsllvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 171, 71, 206], OperandSize::Dword)
}

#[test]
fn vpsllvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1891171897, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 71, 164, 94, 57, 254, 184, 112], OperandSize::Dword)
}

#[test]
fn vpsllvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 1750035967, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 71, 166, 255, 109, 79, 104], OperandSize::Dword)
}

#[test]
fn vpsllvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 5, 174, 71, 204], OperandSize::Qword)
}

#[test]
fn vpsllvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RCX, 376972186, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 13, 173, 71, 177, 154, 35, 120, 22], OperandSize::Qword)
}

#[test]
fn vpsllvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 77, 179, 71, 15], OperandSize::Qword)
}

#[test]
fn vpsllvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 71, 237], OperandSize::Dword)
}

#[test]
fn vpsllvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 639353220, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 71, 148, 192, 132, 193, 27, 38], OperandSize::Dword)
}

#[test]
fn vpsllvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 222, 71, 50], OperandSize::Dword)
}

#[test]
fn vpsllvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 77, 194, 71, 216], OperandSize::Qword)
}

#[test]
fn vpsllvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 990896386, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 37, 194, 71, 180, 145, 2, 225, 15, 59], OperandSize::Qword)
}

#[test]
fn vpsllvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RDI, 496358847, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 29, 220, 71, 175, 191, 213, 149, 29], OperandSize::Qword)
}

