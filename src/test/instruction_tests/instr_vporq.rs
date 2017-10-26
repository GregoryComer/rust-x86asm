use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vporq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 142, 235, 207], OperandSize::Dword)
}

#[test]
fn vporq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1728975076, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 139, 235, 28, 213, 228, 16, 14, 103], OperandSize::Dword)
}

#[test]
fn vporq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 154, 235, 27], OperandSize::Dword)
}

#[test]
fn vporq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 237, 134, 235, 229], OperandSize::Qword)
}

#[test]
fn vporq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RDI, 451358608, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 133, 138, 235, 143, 144, 47, 231, 26], OperandSize::Qword)
}

#[test]
fn vporq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 245, 148, 235, 4, 176], OperandSize::Qword)
}

#[test]
fn vporq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 172, 235, 234], OperandSize::Dword)
}

#[test]
fn vporq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 666236628, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 235, 156, 199, 212, 246, 181, 39], OperandSize::Dword)
}

#[test]
fn vporq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 150132891, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 235, 20, 85, 155, 216, 242, 8], OperandSize::Dword)
}

#[test]
fn vporq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 157, 164, 235, 193], OperandSize::Qword)
}

#[test]
fn vporq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1729094969, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 181, 170, 235, 44, 85, 57, 229, 15, 103], OperandSize::Qword)
}

#[test]
fn vporq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RAX, 77249515, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 253, 179, 235, 184, 235, 187, 154, 4], OperandSize::Qword)
}

#[test]
fn vporq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 202, 235, 227], OperandSize::Dword)
}

#[test]
fn vporq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 201, 235, 12, 130], OperandSize::Dword)
}

#[test]
fn vporq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 217, 235, 60, 88], OperandSize::Dword)
}

#[test]
fn vporq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 165, 201, 235, 225], OperandSize::Qword)
}

#[test]
fn vporq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 660629060, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 197, 196, 235, 12, 149, 68, 102, 96, 39], OperandSize::Qword)
}

#[test]
fn vporq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 189, 210, 235, 55], OperandSize::Qword)
}

