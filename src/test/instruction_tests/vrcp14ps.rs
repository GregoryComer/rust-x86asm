use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 76, 232], OperandSize::Dword)
}

fn vrcp14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 76, 58], OperandSize::Dword)
}

fn vrcp14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 76, 60, 74], OperandSize::Dword)
}

fn vrcp14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 125, 141, 76, 196], OperandSize::Qword)
}

fn vrcp14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RBX, 1088143722, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 76, 179, 106, 193, 219, 64], OperandSize::Qword)
}

fn vrcp14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 506633148, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 157, 76, 180, 67, 188, 155, 50, 30], OperandSize::Qword)
}

fn vrcp14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 76, 200], OperandSize::Dword)
}

fn vrcp14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 76, 34], OperandSize::Dword)
}

fn vrcp14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 76, 60, 246], OperandSize::Dword)
}

fn vrcp14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 125, 169, 76, 199], OperandSize::Qword)
}

fn vrcp14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 317495632, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 175, 76, 140, 201, 80, 153, 236, 18], OperandSize::Qword)
}

fn vrcp14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(YMM25)), operand2: Some(IndirectDisplaced(RSI, 1155034633, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 186, 76, 142, 9, 110, 216, 68], OperandSize::Qword)
}

fn vrcp14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 76, 226], OperandSize::Dword)
}

fn vrcp14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 76, 44, 137], OperandSize::Dword)
}

fn vrcp14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1557570159, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 76, 188, 129, 111, 162, 214, 92], OperandSize::Dword)
}

fn vrcp14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 125, 203, 76, 250], OperandSize::Qword)
}

fn vrcp14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectDisplaced(RCX, 1025816728, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 76, 169, 152, 184, 36, 61], OperandSize::Qword)
}

fn vrcp14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PS, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 399068370, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 223, 76, 132, 241, 210, 76, 201, 23], OperandSize::Qword)
}

