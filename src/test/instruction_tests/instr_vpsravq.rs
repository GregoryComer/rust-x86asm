use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 70, 225], OperandSize::Dword)
}

#[test]
fn vpsravq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 70, 20, 151], OperandSize::Dword)
}

#[test]
fn vpsravq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 154, 70, 48], OperandSize::Dword)
}

#[test]
fn vpsravq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 229, 134, 70, 230], OperandSize::Qword)
}

#[test]
fn vpsravq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 189, 135, 70, 16], OperandSize::Qword)
}

#[test]
fn vpsravq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDI, 783876746, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 197, 154, 70, 191, 138, 2, 185, 46], OperandSize::Qword)
}

#[test]
fn vpsravq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 70, 215], OperandSize::Dword)
}

#[test]
fn vpsravq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 173, 70, 44, 147], OperandSize::Dword)
}

#[test]
fn vpsravq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 191, 70, 12, 191], OperandSize::Dword)
}

#[test]
fn vpsravq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 197, 175, 70, 196], OperandSize::Qword)
}

#[test]
fn vpsravq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 245, 163, 70, 18], OperandSize::Qword)
}

#[test]
fn vpsravq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 186, 70, 36, 64], OperandSize::Qword)
}

#[test]
fn vpsravq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 70, 228], OperandSize::Dword)
}

#[test]
fn vpsravq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 454772247, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 203, 70, 148, 127, 23, 70, 27, 27], OperandSize::Dword)
}

#[test]
fn vpsravq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 223, 70, 28, 154], OperandSize::Dword)
}

#[test]
fn vpsravq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 149, 203, 70, 224], OperandSize::Qword)
}

#[test]
fn vpsravq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1519983827, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 173, 201, 70, 132, 151, 211, 28, 153, 90], OperandSize::Qword)
}

#[test]
fn vpsravq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1293430934, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 210, 70, 156, 64, 150, 48, 24, 77], OperandSize::Qword)
}

