use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pext_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 249], OperandSize::Dword)
}

fn pext_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 245, 36, 135], OperandSize::Dword)
}

fn pext_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 66, 245, 223], OperandSize::Qword)
}

fn pext_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(IndirectDisplaced(RDX, 865150615, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 138, 151, 38, 145, 51], OperandSize::Qword)
}

fn pext_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 210, 245, 247], OperandSize::Qword)
}

fn pext_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RCX)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 899669134, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 242, 245, 148, 94, 142, 220, 159, 53], OperandSize::Qword)
}

