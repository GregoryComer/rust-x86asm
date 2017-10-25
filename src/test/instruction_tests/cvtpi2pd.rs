use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtpi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 207], OperandSize::Word)
}

fn cvtpi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 2], OperandSize::Word)
}

fn cvtpi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 240], OperandSize::Dword)
}

fn cvtpi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 113590888, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 44, 253, 104, 66, 197, 6], OperandSize::Dword)
}

fn cvtpi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 254], OperandSize::Qword)
}

fn cvtpi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 1688335756, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 139, 140, 245, 161, 100], OperandSize::Qword)
}

