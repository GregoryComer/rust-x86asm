use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcastss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 39], OperandSize::Dword)
}

fn vbroadcastss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 44, 185], OperandSize::Qword)
}

fn vbroadcastss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDI, 533284348, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 191, 252, 69, 201, 31], OperandSize::Dword)
}

fn vbroadcastss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 732635222, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 148, 86, 86, 32, 171, 43], OperandSize::Qword)
}

fn vbroadcastss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 217], OperandSize::Dword)
}

fn vbroadcastss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 210], OperandSize::Qword)
}

fn vbroadcastss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 214], OperandSize::Dword)
}

fn vbroadcastss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 209], OperandSize::Qword)
}

fn vbroadcastss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 24, 200], OperandSize::Dword)
}

fn vbroadcastss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 24, 0], OperandSize::Dword)
}

fn vbroadcastss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 141, 24, 193], OperandSize::Qword)
}

fn vbroadcastss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1521209322, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 141, 24, 172, 158, 234, 207, 171, 90], OperandSize::Qword)
}

fn vbroadcastss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 24, 194], OperandSize::Dword)
}

fn vbroadcastss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 24, 12, 216], OperandSize::Dword)
}

fn vbroadcastss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 169, 24, 251], OperandSize::Qword)
}

fn vbroadcastss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM19)), operand2: Some(IndirectDisplaced(RDX, 1319674485, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 174, 24, 154, 117, 162, 168, 78], OperandSize::Qword)
}

fn vbroadcastss_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 24, 205], OperandSize::Dword)
}

fn vbroadcastss_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 24, 52, 191], OperandSize::Dword)
}

fn vbroadcastss_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 204, 24, 239], OperandSize::Qword)
}

fn vbroadcastss_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 170061776, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 24, 132, 143, 208, 239, 34, 10], OperandSize::Qword)
}

