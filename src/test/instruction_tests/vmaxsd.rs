use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 95, 240], OperandSize::Dword)
}

fn vmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 849738650, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 95, 148, 190, 154, 251, 165, 50], OperandSize::Dword)
}

fn vmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 95, 225], OperandSize::Qword)
}

fn vmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 95, 4, 249], OperandSize::Qword)
}

fn vmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 239, 156, 95, 240], OperandSize::Dword)
}

fn vmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1830428606, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 231, 137, 95, 135, 190, 31, 26, 109], OperandSize::Dword)
}

fn vmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 207, 159, 95, 213], OperandSize::Qword)
}

fn vmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RBX, 1140909279, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 191, 142, 95, 179, 223, 228, 0, 68], OperandSize::Qword)
}

