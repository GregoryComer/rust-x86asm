use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaddsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 88, 227], OperandSize::Dword)
}

fn vaddsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 88, 10], OperandSize::Dword)
}

fn vaddsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 88, 207], OperandSize::Qword)
}

fn vaddsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1154722464, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 88, 44, 189, 160, 170, 211, 68], OperandSize::Qword)
}

fn vaddsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 247, 254, 88, 240], OperandSize::Dword)
}

fn vaddsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1668433152, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 199, 143, 88, 28, 125, 0, 69, 114, 99], OperandSize::Dword)
}

fn vaddsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 231, 255, 88, 240], OperandSize::Qword)
}

fn vaddsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 223, 131, 88, 60, 79], OperandSize::Qword)
}

