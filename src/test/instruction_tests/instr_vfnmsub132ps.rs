use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 158, 255], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 158, 23], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 158, 255], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RAX, 1595166503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 158, 128, 39, 79, 20, 95], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 158, 193], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 306764886, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 158, 4, 77, 86, 220, 72, 18], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 158, 216], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1926116752, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 158, 180, 203, 144, 53, 206, 114], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 158, 237], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1585420767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 158, 36, 85, 223, 153, 127, 94], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 686275402, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 155, 158, 159, 74, 187, 231, 40], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 45, 135, 158, 229], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2116307524, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 93, 140, 158, 4, 157, 68, 74, 36, 126], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1271135867, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 13, 157, 158, 44, 197, 123, 254, 195, 75], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 158, 238], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 2050942062, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 158, 60, 189, 110, 228, 62, 122], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 190, 158, 28, 95], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 29, 165, 158, 242], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 280066923, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 5, 162, 158, 164, 192, 107, 123, 177, 16], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RBX, 49022022, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 93, 178, 158, 163, 70, 4, 236, 2], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 217, 158, 233], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EDI, 490443419, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 158, 191, 155, 146, 59, 29], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 158, 38], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 37, 253, 158, 249], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 706838567, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 93, 197, 158, 164, 185, 39, 128, 33, 42], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 109, 209, 158, 42], OperandSize::Qword)
}

