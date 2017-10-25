use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtudq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 122, 210], OperandSize::Dword)
}

fn vcvtudq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1931385584, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 122, 156, 90, 240, 154, 30, 115], OperandSize::Dword)
}

fn vcvtudq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 126, 139, 122, 205], OperandSize::Qword)
}

fn vcvtudq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 126, 141, 122, 52, 240], OperandSize::Qword)
}

fn vcvtudq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 122, 202], OperandSize::Dword)
}

fn vcvtudq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 872752499, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 122, 188, 78, 115, 37, 5, 52], OperandSize::Dword)
}

fn vcvtudq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 126, 169, 122, 239], OperandSize::Qword)
}

fn vcvtudq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 122, 39], OperandSize::Qword)
}

fn vcvtudq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 122, 251], OperandSize::Dword)
}

fn vcvtudq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 205, 122, 44, 147], OperandSize::Dword)
}

fn vcvtudq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 126, 203, 122, 226], OperandSize::Qword)
}

fn vcvtudq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 936365443, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 126, 201, 122, 28, 133, 131, 205, 207, 55], OperandSize::Qword)
}

