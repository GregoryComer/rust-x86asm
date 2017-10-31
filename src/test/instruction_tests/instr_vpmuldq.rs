use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 40, 218], OperandSize::Dword)
}

#[test]
fn vpmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 279410796, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 40, 28, 221, 108, 120, 167, 16], OperandSize::Dword)
}

#[test]
fn vpmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 40, 249], OperandSize::Qword)
}

#[test]
fn vpmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 40, 28, 122], OperandSize::Qword)
}

#[test]
fn vpmuldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 40, 200], OperandSize::Dword)
}

#[test]
fn vpmuldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 97201492, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 40, 52, 125, 84, 45, 203, 5], OperandSize::Dword)
}

#[test]
fn vpmuldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 40, 226], OperandSize::Qword)
}

#[test]
fn vpmuldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDI, 251524654, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 40, 191, 46, 246, 253, 14], OperandSize::Qword)
}

#[test]
fn vpmuldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 40, 239], OperandSize::Dword)
}

#[test]
fn vpmuldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 40, 51], OperandSize::Dword)
}

#[test]
fn vpmuldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 158, 40, 54], OperandSize::Dword)
}

#[test]
fn vpmuldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 205, 135, 40, 232], OperandSize::Qword)
}

#[test]
fn vpmuldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1762883504, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 213, 131, 40, 4, 133, 176, 119, 19, 105], OperandSize::Qword)
}

#[test]
fn vpmuldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 927651991, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 165, 158, 40, 148, 86, 151, 216, 74, 55], OperandSize::Qword)
}

#[test]
fn vpmuldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 40, 239], OperandSize::Dword)
}

#[test]
fn vpmuldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 40, 44, 150], OperandSize::Dword)
}

#[test]
fn vpmuldq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ESI, 439162299, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 191, 40, 150, 187, 21, 45, 26], OperandSize::Dword)
}

#[test]
fn vpmuldq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 133, 161, 40, 217], OperandSize::Qword)
}

#[test]
fn vpmuldq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 173, 173, 40, 20, 218], OperandSize::Qword)
}

#[test]
fn vpmuldq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1643251975, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 229, 177, 40, 180, 70, 7, 9, 242, 97], OperandSize::Qword)
}

#[test]
fn vpmuldq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 40, 209], OperandSize::Dword)
}

#[test]
fn vpmuldq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1879711344, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 206, 40, 180, 217, 112, 30, 10, 112], OperandSize::Dword)
}

#[test]
fn vpmuldq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 220, 40, 57], OperandSize::Dword)
}

#[test]
fn vpmuldq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 205, 194, 40, 212], OperandSize::Qword)
}

#[test]
fn vpmuldq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 197, 198, 40, 52, 222], OperandSize::Qword)
}

#[test]
fn vpmuldq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1479341599, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 141, 218, 40, 20, 181, 31, 246, 44, 88], OperandSize::Qword)
}

