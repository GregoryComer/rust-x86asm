use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 212, 209], OperandSize::Dword)
}

fn vpaddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1863547384, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 140, 246, 248, 121, 19, 111], OperandSize::Dword)
}

fn vpaddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 225], OperandSize::Qword)
}

fn vpaddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 44, 115], OperandSize::Qword)
}

fn vpaddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 212, 201], OperandSize::Dword)
}

fn vpaddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1287955204, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 212, 44, 77, 4, 163, 196, 76], OperandSize::Dword)
}

fn vpaddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 212, 253], OperandSize::Qword)
}

fn vpaddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1846498015, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 212, 180, 199, 223, 82, 15, 110], OperandSize::Qword)
}

fn vpaddq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 142, 212, 247], OperandSize::Dword)
}

fn vpaddq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1446298208, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 212, 4, 221, 96, 194, 52, 86], OperandSize::Dword)
}

fn vpaddq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 464635923, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 154, 212, 184, 19, 200, 177, 27], OperandSize::Dword)
}

fn vpaddq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 205, 129, 212, 195], OperandSize::Qword)
}

fn vpaddq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 173, 131, 212, 28, 79], OperandSize::Qword)
}

fn vpaddq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 1249296437, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 133, 146, 212, 161, 53, 192, 118, 74], OperandSize::Qword)
}

