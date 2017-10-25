use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 59, 250], OperandSize::Dword)
}

fn vpminuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 59, 4, 129], OperandSize::Dword)
}

fn vpminuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1859406257, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 157, 59, 164, 135, 177, 73, 212, 110], OperandSize::Dword)
}

fn vpminuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 253, 141, 59, 237], OperandSize::Qword)
}

fn vpminuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 781251656, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 165, 135, 59, 20, 157, 72, 244, 144, 46], OperandSize::Qword)
}

fn vpminuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 133, 148, 59, 12, 146], OperandSize::Qword)
}

fn vpminuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 59, 231], OperandSize::Dword)
}

fn vpminuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EBX, 891175070, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 59, 155, 158, 64, 30, 53], OperandSize::Dword)
}

fn vpminuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 752070872, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 188, 59, 180, 187, 216, 176, 211, 44], OperandSize::Dword)
}

fn vpminuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 245, 167, 59, 220], OperandSize::Qword)
}

fn vpminuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 869952406, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 149, 173, 59, 20, 157, 150, 107, 218, 51], OperandSize::Qword)
}

fn vpminuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 809627558, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 213, 178, 59, 60, 213, 166, 239, 65, 48], OperandSize::Qword)
}

fn vpminuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 206, 59, 248], OperandSize::Dword)
}

fn vpminuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1359855243, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 59, 132, 246, 139, 190, 13, 81], OperandSize::Dword)
}

fn vpminuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 59, 28, 223], OperandSize::Dword)
}

fn vpminuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 205, 206, 59, 230], OperandSize::Qword)
}

fn vpminuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RSI, 1404742689, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 195, 59, 166, 33, 172, 186, 83], OperandSize::Qword)
}

fn vpminuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 221, 59, 43], OperandSize::Qword)
}

