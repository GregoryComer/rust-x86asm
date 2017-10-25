use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 13, 39, 243], OperandSize::Dword)
}

#[test]
fn vptestmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 2044444495, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 13, 39, 52, 133, 79, 191, 219, 121], OperandSize::Dword)
}

#[test]
fn vptestmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 25, 39, 23], OperandSize::Dword)
}

#[test]
fn vptestmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 197, 7, 39, 251], OperandSize::Qword)
}

#[test]
fn vptestmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1145815887, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 149, 11, 39, 60, 157, 79, 195, 75, 68], OperandSize::Qword)
}

#[test]
fn vptestmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 1647322337, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 29, 39, 150, 225, 36, 48, 98], OperandSize::Qword)
}

#[test]
fn vptestmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 46, 39, 228], OperandSize::Dword)
}

#[test]
fn vptestmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 43, 39, 44, 178], OperandSize::Dword)
}

#[test]
fn vptestmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 743644288, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 61, 39, 52, 69, 128, 28, 83, 44], OperandSize::Dword)
}

#[test]
fn vptestmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 189, 38, 39, 233], OperandSize::Qword)
}

#[test]
fn vptestmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 181, 44, 39, 16], OperandSize::Qword)
}

#[test]
fn vptestmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 919183042, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 59, 39, 140, 87, 194, 158, 201, 54], OperandSize::Qword)
}

#[test]
fn vptestmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 74, 39, 205], OperandSize::Dword)
}

#[test]
fn vptestmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 73, 39, 44, 138], OperandSize::Dword)
}

#[test]
fn vptestmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 91, 39, 52, 72], OperandSize::Dword)
}

#[test]
fn vptestmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 165, 70, 39, 214], OperandSize::Qword)
}

#[test]
fn vptestmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 173, 65, 39, 35], OperandSize::Qword)
}

#[test]
fn vptestmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 81, 39, 25], OperandSize::Qword)
}

