use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 102, 240, 110], OperandSize::Dword)
}

#[test]
fn vfpclassps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 525157773, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 9, 102, 44, 125, 141, 69, 77, 31, 93], OperandSize::Dword)
}

#[test]
fn vfpclassps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 29, 102, 28, 66, 94], OperandSize::Dword)
}

#[test]
fn vfpclassps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM25)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 125, 10, 102, 209, 27], OperandSize::Qword)
}

#[test]
fn vfpclassps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 193696484, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 15, 102, 20, 205, 228, 146, 139, 11, 5], OperandSize::Qword)
}

#[test]
fn vfpclassps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1751554327, Some(OperandSize::Dword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 30, 102, 28, 213, 23, 153, 102, 104, 33], OperandSize::Qword)
}

#[test]
fn vfpclassps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 47, 102, 201, 20], OperandSize::Dword)
}

#[test]
fn vfpclassps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1680978420, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 47, 102, 180, 118, 244, 177, 49, 100, 51], OperandSize::Dword)
}

#[test]
fn vfpclassps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectDisplaced(ESI, 31538305, Some(OperandSize::Dword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 62, 102, 142, 129, 60, 225, 1, 43], OperandSize::Dword)
}

#[test]
fn vfpclassps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 125, 45, 102, 201, 58], OperandSize::Qword)
}

#[test]
fn vfpclassps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 411639771, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 42, 102, 60, 117, 219, 31, 137, 24, 78], OperandSize::Qword)
}

#[test]
fn vfpclassps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 59, 102, 24, 50], OperandSize::Qword)
}

#[test]
fn vfpclassps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 78, 102, 229, 47], OperandSize::Dword)
}

#[test]
fn vfpclassps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 905945901, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 79, 102, 188, 249, 45, 163, 255, 53, 21], OperandSize::Dword)
}

#[test]
fn vfpclassps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 89, 102, 60, 247, 70], OperandSize::Dword)
}

#[test]
fn vfpclassps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 125, 79, 102, 255, 120], OperandSize::Qword)
}

#[test]
fn vfpclassps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 102, 12, 145, 31], OperandSize::Qword)
}

#[test]
fn vfpclassps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 524403105, Some(OperandSize::Dword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 90, 102, 12, 181, 161, 193, 65, 31, 84], OperandSize::Qword)
}

