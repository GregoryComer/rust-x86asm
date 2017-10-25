use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 78, 194], OperandSize::Dword)
}

fn vrsqrt14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1487811037, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 78, 148, 113, 221, 49, 174, 88], OperandSize::Dword)
}

fn vrsqrt14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 585062690, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 78, 36, 93, 34, 89, 223, 34], OperandSize::Dword)
}

fn vrsqrt14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 139, 78, 225], OperandSize::Qword)
}

fn vrsqrt14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1626429492, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 139, 78, 52, 93, 52, 88, 241, 96], OperandSize::Qword)
}

fn vrsqrt14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 153, 78, 52, 159], OperandSize::Qword)
}

fn vrsqrt14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 78, 247], OperandSize::Dword)
}

fn vrsqrt14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 922084332, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 78, 132, 247, 236, 227, 245, 54], OperandSize::Dword)
}

fn vrsqrt14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 207716668, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 191, 78, 12, 149, 60, 129, 97, 12], OperandSize::Dword)
}

fn vrsqrt14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 125, 172, 78, 235], OperandSize::Qword)
}

fn vrsqrt14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 78, 60, 130], OperandSize::Qword)
}

fn vrsqrt14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 191, 78, 28, 145], OperandSize::Qword)
}

fn vrsqrt14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 78, 250], OperandSize::Dword)
}

fn vrsqrt14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1857371936, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 78, 156, 241, 32, 63, 181, 110], OperandSize::Dword)
}

fn vrsqrt14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 78, 57], OperandSize::Dword)
}

fn vrsqrt14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 125, 206, 78, 246], OperandSize::Qword)
}

fn vrsqrt14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 285500686, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 78, 44, 141, 14, 101, 4, 17], OperandSize::Qword)
}

fn vrsqrt14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(RDI, 6947657, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 78, 167, 73, 3, 106, 0], OperandSize::Qword)
}

