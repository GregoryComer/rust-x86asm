use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 64, 233], OperandSize::Dword)
}

#[test]
fn vpmullq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1355387907, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 64, 164, 158, 3, 148, 201, 80], OperandSize::Dword)
}

#[test]
fn vpmullq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 153, 64, 34], OperandSize::Dword)
}

#[test]
fn vpmullq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 205, 129, 64, 247], OperandSize::Qword)
}

#[test]
fn vpmullq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 138, 64, 12, 81], OperandSize::Qword)
}

#[test]
fn vpmullq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 189, 155, 64, 36, 182], OperandSize::Qword)
}

#[test]
fn vpmullq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 64, 210], OperandSize::Dword)
}

#[test]
fn vpmullq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 64, 50], OperandSize::Dword)
}

#[test]
fn vpmullq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 64, 28, 147], OperandSize::Dword)
}

#[test]
fn vpmullq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 245, 166, 64, 253], OperandSize::Qword)
}

#[test]
fn vpmullq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 189, 164, 64, 27], OperandSize::Qword)
}

#[test]
fn vpmullq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 2129873374, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 157, 178, 64, 132, 119, 222, 73, 243, 126], OperandSize::Qword)
}

#[test]
fn vpmullq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 64, 232], OperandSize::Dword)
}

#[test]
fn vpmullq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1511256798, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 64, 44, 197, 222, 242, 19, 90], OperandSize::Dword)
}

#[test]
fn vpmullq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 570701823, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 64, 128, 255, 55, 4, 34], OperandSize::Dword)
}

#[test]
fn vpmullq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 221, 199, 64, 206], OperandSize::Qword)
}

#[test]
fn vpmullq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RSI, 1590190665, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 201, 64, 174, 73, 98, 200, 94], OperandSize::Qword)
}

#[test]
fn vpmullq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 270630970, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 222, 64, 188, 144, 58, 128, 33, 16], OperandSize::Qword)
}

