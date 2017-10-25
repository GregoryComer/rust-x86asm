use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 203], OperandSize::Dword)
}

fn cvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 659989796, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 20, 253, 36, 165, 86, 39], OperandSize::Dword)
}

fn cvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 223], OperandSize::Qword)
}

fn cvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1350100286, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 45, 188, 139, 62, 229, 120, 80], OperandSize::Qword)
}

fn cvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 254], OperandSize::Qword)
}

fn cvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 73208364, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 45, 60, 125, 44, 18, 93, 4], OperandSize::Qword)
}

