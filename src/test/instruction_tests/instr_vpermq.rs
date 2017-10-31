use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 54, 198], OperandSize::Dword)
}

#[test]
fn vpermq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 480021502, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 54, 151, 254, 139, 156, 28], OperandSize::Dword)
}

#[test]
fn vpermq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1329511810, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 188, 54, 44, 197, 130, 189, 62, 79], OperandSize::Dword)
}

#[test]
fn vpermq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 141, 175, 54, 230], OperandSize::Qword)
}

#[test]
fn vpermq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 213, 170, 54, 60, 154], OperandSize::Qword)
}

#[test]
fn vpermq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 237, 191, 54, 50], OperandSize::Qword)
}

#[test]
fn vpermq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 54, 243], OperandSize::Dword)
}

#[test]
fn vpermq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 54, 28, 119], OperandSize::Dword)
}

#[test]
fn vpermq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 1404640675, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 221, 54, 146, 163, 29, 185, 83], OperandSize::Dword)
}

#[test]
fn vpermq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 141, 194, 54, 235], OperandSize::Qword)
}

#[test]
fn vpermq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2138934141, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 229, 201, 54, 52, 93, 125, 139, 125, 127], OperandSize::Qword)
}

#[test]
fn vpermq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 415291602, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 211, 54, 44, 85, 210, 216, 192, 24], OperandSize::Qword)
}

#[test]
fn vpermq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 207, 83], OperandSize::Dword)
}

#[test]
fn vpermq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 183057807, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 132, 134, 143, 61, 233, 10, 74], OperandSize::Dword)
}

#[test]
fn vpermq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 231, 49], OperandSize::Qword)
}

#[test]
fn vpermq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 0, 108], OperandSize::Qword)
}

#[test]
fn vpermq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 0, 193, 2], OperandSize::Dword)
}

#[test]
fn vpermq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 2004496186, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 0, 140, 243, 58, 47, 122, 119, 65], OperandSize::Dword)
}

#[test]
fn vpermq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1414041090, Some(OperandSize::Qword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 0, 172, 247, 2, 142, 72, 84, 120], OperandSize::Dword)
}

#[test]
fn vpermq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 253, 173, 0, 206, 36], OperandSize::Qword)
}

#[test]
fn vpermq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RSI, 1976557473, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 170, 0, 158, 161, 223, 207, 117, 42], OperandSize::Qword)
}

#[test]
fn vpermq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1593034785, Some(OperandSize::Qword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 0, 156, 78, 33, 200, 243, 94, 35], OperandSize::Qword)
}

#[test]
fn vpermq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 0, 222, 3], OperandSize::Dword)
}

#[test]
fn vpermq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 0, 52, 184, 75], OperandSize::Dword)
}

#[test]
fn vpermq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 183393592, Some(OperandSize::Qword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 218, 0, 28, 133, 56, 93, 238, 10, 124], OperandSize::Dword)
}

#[test]
fn vpermq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 253, 205, 0, 217, 48], OperandSize::Qword)
}

#[test]
fn vpermq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM18)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 253, 205, 0, 17, 16], OperandSize::Qword)
}

#[test]
fn vpermq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 66528624, Some(OperandSize::Qword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 0, 52, 149, 112, 37, 247, 3, 88], OperandSize::Qword)
}

