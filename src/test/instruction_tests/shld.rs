use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn shld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 247, 59], OperandSize::Word)
}

fn shld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 61, 7], OperandSize::Word)
}

fn shld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 255, 95], OperandSize::Dword)
}

fn shld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(ESI, 19080880, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 158, 176, 38, 35, 1, 72], OperandSize::Dword)
}

fn shld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 205, 58], OperandSize::Qword)
}

fn shld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1330350633, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 36, 213, 41, 138, 75, 79, 75], OperandSize::Qword)
}

fn shld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 220, 82], OperandSize::Word)
}

fn shld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 180, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 178, 180, 0, 42], OperandSize::Word)
}

fn shld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 217, 6], OperandSize::Dword)
}

fn shld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(EBX, 1565865008, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 139, 48, 52, 85, 93, 2], OperandSize::Dword)
}

fn shld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 220, 85], OperandSize::Qword)
}

fn shld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(RDI, 326236214, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 183, 54, 248, 113, 19, 9], OperandSize::Qword)
}

fn shld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBX)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 218, 67], OperandSize::Qword)
}

fn shld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 2027010056, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 140, 82, 8, 184, 209, 120, 119], OperandSize::Qword)
}

fn shld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 219], OperandSize::Word)
}

fn shld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(BP, 247, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 166, 247, 0], OperandSize::Word)
}

fn shld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 202], OperandSize::Dword)
}

fn shld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 31], OperandSize::Dword)
}

fn shld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 226], OperandSize::Qword)
}

fn shld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 43], OperandSize::Qword)
}

fn shld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 210], OperandSize::Word)
}

fn shld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 118, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 72, 118], OperandSize::Word)
}

fn shld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 221], OperandSize::Dword)
}

fn shld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(ECX, 1297287687, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 153, 7, 10, 83, 77], OperandSize::Dword)
}

fn shld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 242], OperandSize::Qword)
}

fn shld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 52, 209], OperandSize::Qword)
}

fn shld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 212], OperandSize::Qword)
}

fn shld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 33], OperandSize::Qword)
}

