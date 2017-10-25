use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 139, 91, 219], OperandSize::Dword)
}

fn vcvtqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 138, 91, 11], OperandSize::Dword)
}

fn vcvtqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 252, 142, 91, 219], OperandSize::Qword)
}

fn vcvtqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2116699309, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 91, 52, 157, 173, 68, 42, 126], OperandSize::Qword)
}

fn vcvtqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 91, 222], OperandSize::Dword)
}

fn vcvtqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1695663478, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 169, 91, 60, 245, 118, 197, 17, 101], OperandSize::Dword)
}

fn vcvtqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 252, 169, 91, 199], OperandSize::Qword)
}

fn vcvtqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM21)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 252, 175, 91, 41], OperandSize::Qword)
}

fn vcvtqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 250, 91, 216], OperandSize::Dword)
}

fn vcvtqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 203, 91, 23], OperandSize::Dword)
}

fn vcvtqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 252, 158, 91, 226], OperandSize::Qword)
}

fn vcvtqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 2127924888, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 252, 201, 91, 180, 223, 152, 142, 213, 126], OperandSize::Qword)
}

