use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(DI)), operand2: Some(Memory(12797, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 62, 253, 49], OperandSize::Word)
}

fn movbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 97130238, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 28, 157, 254, 22, 202, 5], OperandSize::Dword)
}

fn movbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 135359188, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 148, 201, 212, 106, 17, 8], OperandSize::Qword)
}

fn movbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 193, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 179, 193, 0], OperandSize::Word)
}

fn movbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 31], OperandSize::Dword)
}

fn movbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 58], OperandSize::Qword)
}

fn movbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 240, 36, 159], OperandSize::Qword)
}

fn movbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 34], OperandSize::Word)
}

fn movbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 18], OperandSize::Dword)
}

fn movbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledDisplaced(RAX, Two, 258670996, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 20, 69, 148, 1, 107, 15], OperandSize::Qword)
}

fn movbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 21], OperandSize::Word)
}

fn movbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 60, 71], OperandSize::Dword)
}

fn movbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 832615118, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 180, 91, 206, 178, 160, 49], OperandSize::Qword)
}

fn movbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 241, 28, 242], OperandSize::Qword)
}

