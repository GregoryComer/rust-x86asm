use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetexpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 66, 255], OperandSize::Dword)
}

fn vgetexpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 759928529, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 66, 28, 117, 209, 150, 75, 45], OperandSize::Dword)
}

fn vgetexpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 66, 60, 223], OperandSize::Dword)
}

fn vgetexpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 138, 66, 240], OperandSize::Qword)
}

fn vgetexpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 141, 66, 28, 154], OperandSize::Qword)
}

fn vgetexpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1651575151, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 125, 157, 66, 28, 213, 111, 9, 113, 98], OperandSize::Qword)
}

fn vgetexpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 66, 223], OperandSize::Dword)
}

fn vgetexpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 479530485, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 66, 132, 223, 245, 13, 149, 28], OperandSize::Dword)
}

fn vgetexpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 676832424, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 66, 12, 213, 168, 164, 87, 40], OperandSize::Dword)
}

fn vgetexpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 125, 173, 66, 236], OperandSize::Qword)
}

fn vgetexpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 66, 20, 155], OperandSize::Qword)
}

fn vgetexpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 191, 66, 44, 80], OperandSize::Qword)
}

fn vgetexpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 66, 201], OperandSize::Dword)
}

fn vgetexpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 780398667, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 66, 148, 201, 75, 240, 131, 46], OperandSize::Dword)
}

fn vgetexpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1134187516, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 66, 188, 146, 252, 83, 154, 67], OperandSize::Dword)
}

fn vgetexpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 125, 154, 66, 208], OperandSize::Qword)
}

fn vgetexpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1792860879, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 66, 156, 182, 207, 226, 220, 106], OperandSize::Qword)
}

fn vgetexpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 218, 66, 4, 217], OperandSize::Qword)
}

