use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtudq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 143, 122, 213], OperandSize::Dword)
}

fn vcvtudq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 137, 122, 36, 178], OperandSize::Dword)
}

fn vcvtudq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 127, 141, 122, 204], OperandSize::Qword)
}

fn vcvtudq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1514048537, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 138, 122, 140, 246, 25, 140, 62, 90], OperandSize::Qword)
}

fn vcvtudq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 122, 228], OperandSize::Dword)
}

fn vcvtudq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 2023798305, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 122, 148, 128, 33, 182, 160, 120], OperandSize::Dword)
}

fn vcvtudq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 127, 170, 122, 236], OperandSize::Qword)
}

fn vcvtudq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RBX, 1513806576, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 122, 163, 240, 218, 58, 90], OperandSize::Qword)
}

fn vcvtudq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 155, 122, 192], OperandSize::Dword)
}

fn vcvtudq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 205, 122, 60, 248], OperandSize::Dword)
}

fn vcvtudq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 127, 153, 122, 253], OperandSize::Qword)
}

fn vcvtudq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM18)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 127, 202, 122, 22], OperandSize::Qword)
}

