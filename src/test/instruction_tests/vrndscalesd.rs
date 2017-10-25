use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrndscalesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 156, 11, 222, 60], OperandSize::Dword)
}

fn vrndscalesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 140, 11, 36, 183, 74], OperandSize::Dword)
}

fn vrndscalesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 51, 173, 146, 11, 232, 51], OperandSize::Qword)
}

fn vrndscalesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1268647259, Some(OperandSize::Qword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 205, 132, 11, 12, 77, 91, 5, 158, 75, 73], OperandSize::Qword)
}

