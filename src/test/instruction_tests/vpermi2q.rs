use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermi2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 118, 216], OperandSize::Dword)
}

fn vpermi2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1919494435, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 118, 132, 241, 35, 41, 105, 114], OperandSize::Dword)
}

fn vpermi2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 662528095, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 158, 118, 180, 130, 95, 96, 125, 39], OperandSize::Dword)
}

fn vpermi2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 157, 137, 118, 211], OperandSize::Qword)
}

fn vpermi2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RDX, 24829924, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 131, 118, 154, 228, 223, 122, 1], OperandSize::Qword)
}

fn vpermi2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 149, 157, 118, 62], OperandSize::Qword)
}

fn vpermi2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 118, 192], OperandSize::Dword)
}

fn vpermi2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 1523516263, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 118, 166, 103, 3, 207, 90], OperandSize::Dword)
}

fn vpermi2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDI, 1954073392, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 189, 118, 183, 48, 203, 120, 116], OperandSize::Dword)
}

fn vpermi2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 181, 162, 118, 204], OperandSize::Qword)
}

fn vpermi2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1923069269, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 165, 164, 118, 60, 125, 85, 181, 159, 114], OperandSize::Qword)
}

fn vpermi2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1578366098, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 149, 177, 118, 12, 157, 146, 244, 19, 94], OperandSize::Qword)
}

fn vpermi2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 118, 196], OperandSize::Dword)
}

fn vpermi2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1498407661, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 205, 118, 172, 194, 237, 226, 79, 89], OperandSize::Dword)
}

fn vpermi2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EAX, 1762372823, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 219, 118, 176, 215, 172, 11, 105], OperandSize::Dword)
}

fn vpermi2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 149, 206, 118, 251], OperandSize::Qword)
}

fn vpermi2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1337132301, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 229, 193, 118, 156, 195, 13, 5, 179, 79], OperandSize::Qword)
}

fn vpermi2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 205, 219, 118, 36, 251], OperandSize::Qword)
}

