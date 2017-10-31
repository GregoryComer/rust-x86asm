use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 54, 236], OperandSize::Dword)
}

#[test]
fn vpermq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 119521530, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 54, 137, 250, 192, 31, 7], OperandSize::Dword)
}

#[test]
fn vpermq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1019461151, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 186, 54, 177, 31, 190, 195, 60], OperandSize::Dword)
}

#[test]
fn vpermq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 133, 163, 54, 254], OperandSize::Qword)
}

#[test]
fn vpermq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDI, 1059031061, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 175, 54, 183, 21, 136, 31, 63], OperandSize::Qword)
}

#[test]
fn vpermq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 213, 181, 54, 24], OperandSize::Qword)
}

#[test]
fn vpermq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 54, 212], OperandSize::Dword)
}

#[test]
fn vpermq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1204227585, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 54, 4, 141, 1, 14, 199, 71], OperandSize::Dword)
}

#[test]
fn vpermq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1412199592, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 220, 54, 12, 149, 168, 116, 44, 84], OperandSize::Dword)
}

#[test]
fn vpermq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 197, 198, 54, 236], OperandSize::Qword)
}

#[test]
fn vpermq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1555532860, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 141, 205, 54, 20, 125, 60, 140, 183, 92], OperandSize::Qword)
}

#[test]
fn vpermq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 133, 219, 54, 44, 211], OperandSize::Qword)
}

#[test]
fn vpermq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 246, 81], OperandSize::Dword)
}

#[test]
fn vpermq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 60, 177, 51], OperandSize::Dword)
}

#[test]
fn vpermq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 218, 54], OperandSize::Qword)
}

#[test]
fn vpermq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1390498943, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 132, 142, 127, 84, 225, 82, 7], OperandSize::Qword)
}

#[test]
fn vpermq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 0, 198, 42], OperandSize::Dword)
}

#[test]
fn vpermq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 251410723, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 0, 52, 245, 35, 57, 252, 14, 62], OperandSize::Dword)
}

#[test]
fn vpermq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(ECX, 1744891747, Some(OperandSize::Qword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 0, 177, 99, 239, 0, 104, 28], OperandSize::Dword)
}

#[test]
fn vpermq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM12)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 253, 174, 0, 212, 114], OperandSize::Qword)
}

#[test]
fn vpermq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 253, 172, 0, 36, 129, 75], OperandSize::Qword)
}

#[test]
fn vpermq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 191, 0, 36, 66, 110], OperandSize::Qword)
}

#[test]
fn vpermq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 0, 199, 105], OperandSize::Dword)
}

#[test]
fn vpermq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 0, 28, 147, 31], OperandSize::Dword)
}

#[test]
fn vpermq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 2072617110, Some(OperandSize::Qword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 222, 0, 172, 218, 150, 160, 137, 123, 16], OperandSize::Dword)
}

#[test]
fn vpermq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM23)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 253, 206, 0, 223, 67], OperandSize::Qword)
}

#[test]
fn vpermq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 387357195, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 0, 164, 134, 11, 154, 22, 23, 8], OperandSize::Qword)
}

#[test]
fn vpermq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectDisplaced(RBX, 325311106, Some(OperandSize::Qword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 217, 0, 147, 130, 218, 99, 19, 82], OperandSize::Qword)
}

