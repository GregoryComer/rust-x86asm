use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 109, 229], OperandSize::Dword)
}

fn vpunpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 109, 56], OperandSize::Dword)
}

fn vpunpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 109, 223], OperandSize::Qword)
}

fn vpunpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 109, 9], OperandSize::Qword)
}

fn vpunpckhqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 109, 195], OperandSize::Dword)
}

fn vpunpckhqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 1124871237, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 109, 147, 69, 44, 12, 67], OperandSize::Dword)
}

fn vpunpckhqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 109, 220], OperandSize::Qword)
}

fn vpunpckhqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 109, 40], OperandSize::Qword)
}

fn vpunpckhqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 109, 230], OperandSize::Dword)
}

fn vpunpckhqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 834889481, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 109, 44, 93, 9, 103, 195, 49], OperandSize::Dword)
}

fn vpunpckhqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1008717348, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 154, 109, 187, 36, 206, 31, 60], OperandSize::Dword)
}

fn vpunpckhqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 173, 137, 109, 213], OperandSize::Qword)
}

fn vpunpckhqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 893838683, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 142, 109, 180, 138, 91, 229, 70, 53], OperandSize::Qword)
}

fn vpunpckhqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 213, 154, 109, 58], OperandSize::Qword)
}

