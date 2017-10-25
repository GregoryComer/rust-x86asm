use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsravq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 70, 253], OperandSize::Dword)
}

fn vpsravq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1396106492, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 70, 12, 141, 252, 228, 54, 83], OperandSize::Dword)
}

fn vpsravq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 70, 14], OperandSize::Dword)
}

fn vpsravq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 181, 139, 70, 205], OperandSize::Qword)
}

fn vpsravq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 2099550537, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 70, 140, 200, 73, 153, 36, 125], OperandSize::Qword)
}

fn vpsravq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 149, 153, 70, 10], OperandSize::Qword)
}

fn vpsravq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 170, 70, 231], OperandSize::Dword)
}

fn vpsravq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 70, 0], OperandSize::Dword)
}

fn vpsravq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 699698702, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 186, 70, 180, 177, 14, 142, 180, 41], OperandSize::Dword)
}

fn vpsravq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 173, 165, 70, 210], OperandSize::Qword)
}

fn vpsravq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 245, 175, 70, 52, 145], OperandSize::Qword)
}

fn vpsravq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 388202890, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 187, 70, 164, 159, 138, 129, 35, 23], OperandSize::Qword)
}

fn vpsravq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 70, 247], OperandSize::Dword)
}

fn vpsravq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 117062784, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 205, 70, 52, 77, 128, 60, 250, 6], OperandSize::Dword)
}

fn vpsravq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 603354300, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 223, 70, 164, 82, 188, 116, 246, 35], OperandSize::Dword)
}

fn vpsravq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 181, 205, 70, 210], OperandSize::Qword)
}

fn vpsravq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1257877311, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 197, 199, 70, 36, 117, 63, 175, 249, 74], OperandSize::Qword)
}

fn vpsravq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RCX, 795162530, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 213, 219, 70, 161, 162, 55, 101, 47], OperandSize::Qword)
}

