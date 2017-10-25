use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 77, 218], OperandSize::Dword)
}

fn vrcp14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 77, 46], OperandSize::Dword)
}

fn vrcp14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 117, 133, 77, 253], OperandSize::Qword)
}

fn vrcp14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDX, 830348177, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 131, 77, 146, 145, 27, 126, 49], OperandSize::Qword)
}

