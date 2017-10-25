use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprolvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 21, 228], OperandSize::Dword)
}

fn vprolvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 469515462, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 21, 179, 198, 60, 252, 27], OperandSize::Dword)
}

fn vprolvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 538395250, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 158, 21, 4, 117, 114, 66, 23, 32], OperandSize::Dword)
}

fn vprolvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 173, 139, 21, 232], OperandSize::Qword)
}

fn vprolvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 875823568, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 149, 132, 21, 188, 144, 208, 1, 52, 52], OperandSize::Qword)
}

fn vprolvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1048122510, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 221, 150, 21, 12, 149, 142, 20, 121, 62], OperandSize::Qword)
}

fn vprolvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 21, 244], OperandSize::Dword)
}

fn vprolvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 21, 36, 89], OperandSize::Dword)
}

fn vprolvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1131204113, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 21, 134, 17, 206, 108, 67], OperandSize::Dword)
}

fn vprolvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 189, 172, 21, 206], OperandSize::Qword)
}

fn vprolvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 229, 173, 21, 15], OperandSize::Qword)
}

fn vprolvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 315865158, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 178, 21, 148, 138, 70, 184, 211, 18], OperandSize::Qword)
}

fn vprolvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 205, 21, 231], OperandSize::Dword)
}

fn vprolvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 674119569, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 206, 21, 180, 145, 145, 63, 46, 40], OperandSize::Dword)
}

fn vprolvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 21, 4, 90], OperandSize::Dword)
}

fn vprolvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 189, 204, 21, 223], OperandSize::Qword)
}

fn vprolvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 542534516, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 181, 197, 21, 20, 221, 116, 107, 86, 32], OperandSize::Qword)
}

fn vprolvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RBX, 929848640, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 229, 215, 21, 179, 64, 93, 108, 55], OperandSize::Qword)
}

