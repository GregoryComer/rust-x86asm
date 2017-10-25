use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermi2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 117, 236], OperandSize::Dword)
}

fn vpermi2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 117, 30], OperandSize::Dword)
}

fn vpermi2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 213, 131, 117, 217], OperandSize::Qword)
}

fn vpermi2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 221, 129, 117, 28, 74], OperandSize::Qword)
}

fn vpermi2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 117, 234], OperandSize::Dword)
}

fn vpermi2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 117, 55], OperandSize::Dword)
}

fn vpermi2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 253, 171, 117, 224], OperandSize::Qword)
}

fn vpermi2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 2051382820, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 181, 175, 117, 156, 151, 36, 158, 69, 122], OperandSize::Qword)
}

fn vpermi2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 117, 245], OperandSize::Dword)
}

fn vpermi2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 680598996, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 202, 117, 28, 117, 212, 29, 145, 40], OperandSize::Dword)
}

fn vpermi2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 221, 207, 117, 247], OperandSize::Qword)
}

fn vpermi2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 117, 26], OperandSize::Qword)
}

