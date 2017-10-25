use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 119, 240], OperandSize::Dword)
}

fn vpermi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 720120455, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 119, 4, 221, 135, 42, 236, 42], OperandSize::Dword)
}

fn vpermi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 898787168, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 157, 119, 172, 153, 96, 103, 146, 53], OperandSize::Dword)
}

fn vpermi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 132, 119, 221], OperandSize::Qword)
}

fn vpermi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1467798112, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 189, 143, 119, 172, 219, 96, 210, 124, 87], OperandSize::Qword)
}

fn vpermi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RDI, 343056109, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 149, 151, 119, 191, 237, 158, 114, 20], OperandSize::Qword)
}

fn vpermi2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 119, 255], OperandSize::Dword)
}

fn vpermi2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 119, 59], OperandSize::Dword)
}

fn vpermi2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 100009642, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 186, 119, 4, 85, 170, 6, 246, 5], OperandSize::Dword)
}

fn vpermi2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 133, 167, 119, 254], OperandSize::Qword)
}

fn vpermi2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 119, 49], OperandSize::Qword)
}

fn vpermi2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectDisplaced(RSI, 560489840, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 183, 119, 174, 112, 101, 104, 33], OperandSize::Qword)
}

fn vpermi2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 119, 228], OperandSize::Dword)
}

fn vpermi2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 119, 60, 210], OperandSize::Dword)
}

fn vpermi2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1408814913, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 217, 119, 148, 134, 65, 207, 248, 83], OperandSize::Dword)
}

fn vpermi2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 229, 197, 119, 211], OperandSize::Qword)
}

fn vpermi2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDI, 1860961209, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 133, 202, 119, 151, 185, 3, 236, 110], OperandSize::Qword)
}

fn vpermi2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RBX, 709208245, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 213, 213, 119, 187, 181, 168, 69, 42], OperandSize::Qword)
}

