use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 138, 230, 224], OperandSize::Dword)
}

fn vcvtqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1550684978, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 142, 230, 20, 133, 50, 147, 109, 92], OperandSize::Dword)
}

fn vcvtqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 254, 140, 230, 218], OperandSize::Qword)
}

fn vcvtqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 254, 140, 230, 47], OperandSize::Qword)
}

fn vcvtqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 230, 211], OperandSize::Dword)
}

fn vcvtqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 230, 63], OperandSize::Dword)
}

fn vcvtqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 254, 175, 230, 215], OperandSize::Qword)
}

fn vcvtqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM23)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 254, 172, 230, 59], OperandSize::Qword)
}

fn vcvtqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 217, 230, 244], OperandSize::Dword)
}

fn vcvtqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 583027334, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 206, 230, 140, 91, 134, 74, 192, 34], OperandSize::Dword)
}

fn vcvtqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 254, 254, 230, 216], OperandSize::Qword)
}

fn vcvtqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 2065047377, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 254, 202, 230, 4, 141, 81, 31, 22, 123], OperandSize::Qword)
}

