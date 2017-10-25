use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 78, 255], OperandSize::Dword)
}

fn vrsqrt14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 78, 50], OperandSize::Dword)
}

fn vrsqrt14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 156, 78, 12, 243], OperandSize::Dword)
}

fn vrsqrt14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 78, 202], OperandSize::Qword)
}

fn vrsqrt14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 31234726, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 142, 78, 132, 216, 166, 154, 220, 1], OperandSize::Qword)
}

fn vrsqrt14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 78, 63], OperandSize::Qword)
}

fn vrsqrt14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 78, 240], OperandSize::Dword)
}

fn vrsqrt14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ECX, 1752168316, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 78, 185, 124, 247, 111, 104], OperandSize::Dword)
}

fn vrsqrt14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 78, 46], OperandSize::Dword)
}

fn vrsqrt14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 171, 78, 249], OperandSize::Qword)
}

fn vrsqrt14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 749341906, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 171, 78, 52, 213, 210, 12, 170, 44], OperandSize::Qword)
}

fn vrsqrt14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 188, 78, 52, 215], OperandSize::Qword)
}

fn vrsqrt14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 78, 213], OperandSize::Dword)
}

fn vrsqrt14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDX, 707891079, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 78, 186, 135, 143, 49, 42], OperandSize::Dword)
}

fn vrsqrt14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 78, 57], OperandSize::Dword)
}

fn vrsqrt14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 253, 207, 78, 208], OperandSize::Qword)
}

fn vrsqrt14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 78, 2], OperandSize::Qword)
}

fn vrsqrt14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 78, 9], OperandSize::Qword)
}

