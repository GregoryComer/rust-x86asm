use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdivss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 94, 239], OperandSize::Dword)
}

fn vdivss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1681534832, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 94, 52, 245, 112, 47, 58, 100], OperandSize::Dword)
}

fn vdivss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 94, 221], OperandSize::Qword)
}

fn vdivss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 94, 2], OperandSize::Qword)
}

fn vdivss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 70, 186, 94, 195], OperandSize::Dword)
}

fn vdivss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 791631464, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 70, 143, 94, 36, 197, 104, 86, 47, 47], OperandSize::Dword)
}

fn vdivss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 22, 212, 94, 234], OperandSize::Qword)
}

fn vdivss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 22, 130, 94, 48], OperandSize::Qword)
}

